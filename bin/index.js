#!/usr/bin/env node

// index.ts
import fs from "fs-extra";
import { fileURLToPath } from "url";
import path from "path";
import inquirer from "inquirer";
import chalk from "chalk";

// utils/template.ts
import * as ejs from "ejs";
function render2(content, data) {
  return ejs.render(content, data);
}

// index.ts
import shell from "shelljs";
import ora from "ora";
var __filename = fileURLToPath(import.meta.url);
var __dirname = path.dirname(__filename);
var CHOICES = fs.readdirSync(path.join(__dirname, "templates"));
var QUESTIONS = [
  {
    name: "template",
    type: "list",
    message: "What template would you like to use?",
    choices: CHOICES
  },
  {
    name: "name",
    type: "input",
    message: "Please input a new project name:"
  }
];
var CURR_DIR = process.cwd();
var prompt = inquirer.createPromptModule();
prompt(QUESTIONS).then((answers) => {
  const projectChoice = answers["template"];
  const projectName = answers["name"];
  const templatePath = path.join(__dirname, "templates", projectChoice);
  const targetPath = path.join(CURR_DIR, projectName);
  const options = {
    projectName,
    templateName: projectChoice,
    templatePath,
    targetPath
  };
  if (!createProject(targetPath)) {
    return;
  }
  const spinner = ora("Loading templates...").start();
  spinner.spinner = "arrow3";
  setTimeout(() => {
    createDirectoryContents(templatePath, projectName);
    postProcess(options);
    spinner.succeed();
  }, 7e3);
});
function createProject(projectPath) {
  if (fs.existsSync(projectPath)) {
    console.log(chalk.red(`Folder ${projectPath} exists. Delete or use another name.`));
    return false;
  }
  fs.mkdirSync(projectPath);
  return true;
}
var SKIP_FILES = ["node_modules", ".template.json"];
function createDirectoryContents(templatePath, projectName) {
  const filesToCreate = fs.readdirSync(templatePath);
  filesToCreate.forEach((file) => {
    const origFilePath = path.join(templatePath, file);
    const stats = fs.statSync(origFilePath);
    if (SKIP_FILES.indexOf(file) > -1)
      return;
    if (stats.isFile()) {
      let contents = fs.readFileSync(origFilePath, "utf8");
      contents = render2(contents, { projectName });
      const writePath = path.join(CURR_DIR, projectName, file);
      fs.writeFileSync(writePath, contents, "utf8");
    } else if (stats.isDirectory()) {
      fs.mkdirSync(path.join(CURR_DIR, projectName, file));
      createDirectoryContents(path.join(templatePath, file), path.join(projectName, file));
    }
  });
}
function postProcess(options) {
  const isNode = fs.existsSync(path.join(options.targetPath, "package.json"));
  if (isNode) {
    shell.cd(options.targetPath);
    const result = shell.exec("npm install");
    if (result.code !== 0) {
      return false;
    }
  }
  const gitignoreExists = fs.existsSync(path.join(options.targetPath, "_gitignore"));
  if (gitignoreExists) {
    shell.cd(options.targetPath);
    const result = shell.exec("mv _gitignore .gitignore");
    if (result.code !== 0) {
      return false;
    }
  }
  return true;
}
//# sourceMappingURL=index.js.map