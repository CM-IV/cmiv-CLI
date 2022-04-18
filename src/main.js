import chalk from 'chalk';
import fs from 'fs';
import ncp from 'ncp';
import path from 'path';
import { promisify } from 'util';
import execa from 'execa';
import Listr from 'listr';
import { projectInstall } from 'pkg-install';

const access = promisify(fs.access);
const copy = promisify(ncp);


async function gitIgnoreFile(err) {
  const gitignoreExists = fs.existsSync(path.join(__dirname, '.gitignore'));
  if (gitignoreExists) {
    // Append if there's already a `.gitignore` file there
    const data = fs.readFileSync(path.join(__dirname, 'gitignore'));
    fs.appendFileSync(path.join(__dirname, '.gitignore'), data);
    fs.unlinkSync(path.join(__dirname, 'gitignore'));
  } else {
    // Rename gitignore after the fact to prevent npm from renaming it to .npmignore
    // See: https://github.com/npm/npm/issues/1862
    const stream = fs.createWriteStream(".gitignore");
    stream.once('open', () => {
        stream.write('node_modules\n');
        stream.write('build\n');
        stream.write('coverage\n');
        stream.write('.vscode\n');
        stream.write('.env\n');
        stream.write('tmp\n');
        stream.end();
    })

    if (err) throw err;
  }
}

async function copyTemplateFiles(options) {

  return copy(options.templateDirectory, options.targetDirectory, {
    clobber: false,
  });
}

async function initGit(options) {
 const result = await execa('git', ['init'], {
   cwd: options.targetDirectory,
 });
 if (result.failed) {
   return Promise.reject(new Error('Failed to initialize git'));
 }
 return;
}

export async function generateProject(options) {
 options = {
   ...options,
   targetDirectory: options.targetDirectory || process.cwd()
 };

 const templateDir = path.resolve(
   new URL(import.meta.url).pathname,
   '../../templates',
   options.template
 );
 options.templateDirectory = templateDir;

 try {
   await access(templateDir, fs.constants.R_OK);
 } catch (err) {
   console.error('%s Invalid template name', chalk.red.bold('ERROR'));
   console.log(err);
   process.exit(1);
   
 }

 const tasks = new Listr([
   {
     title: 'Check for gitignore file',
     task: () => gitIgnoreFile(),
   },
   {
     title: 'Copy project files',
     task: () => copyTemplateFiles(options),
   },
   {
     title: 'Initialize git',
     task: () => initGit(options),
     enabled: () => options.git,
   },
   {
     title: 'Install dependencies',
     task: () =>
       projectInstall({
         cwd: options.targetDirectory,
       }),
     skip: () =>
       !options.runInstall
         ? 'Pass --install to automatically install dependencies'
         : undefined,
   },
 ]);

 await tasks.run();
 console.log('%s Project ready', chalk.green.bold('DONE'));
 return true;
}
