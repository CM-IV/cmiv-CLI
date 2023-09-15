#!/usr/bin/env node

// index.ts
import fs from "fs-extra";
import path from "pathe";
import shell from "shelljs";
import consola from "consola";
import { normalizeURL, withoutProtocol } from "ufo";
var __filename = withoutProtocol(normalizeURL(import.meta.url));
var __dirname = path.dirname(__filename);
var CHOICES = fs.readdirSync(path.join(__dirname, "templates"));
var CURR_DIR = process.cwd();
consola.box({
  title: "CMIV-CLI",
  message: `Thanks for using this tool!`,
  style: {
    padding: 1,
    borderColor: "magenta",
    borderStyle: "double-single-rounded"
  }
});
consola.prompt("What template would you like to use?", {
  type: "select",
  options: CHOICES
}).then(async (projectChoice) => {
  const projectName = await consola.prompt("Please give the project directory name", { type: "text" });
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
  consola.start("Loading templates...");
  createDirectoryContents(templatePath, projectName);
  postProcess(options);
  consola.success("Template loaded!");
});
function createProject(projectPath) {
  if (fs.existsSync(projectPath)) {
    consola.error(`Folder ${projectPath} exists. Delete or use another name.`);
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