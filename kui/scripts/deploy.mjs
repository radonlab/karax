import path from 'path';
import shell from 'shelljs';
import { projectDir } from './utils.mjs';

const distDir = path.resolve(projectDir, 'dist');
const staticDir = path.resolve(projectDir, '..', 'static');

shell.mkdir('-p', path.resolve(staticDir, 'js'));
shell.mkdir('-p', path.resolve(staticDir, 'css'));
shell.cp(path.resolve(distDir, '*.js'), path.resolve(staticDir, 'js'));
shell.cp(path.resolve(distDir, '*.css'), path.resolve(staticDir, 'css'));
