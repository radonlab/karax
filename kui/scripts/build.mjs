import path from 'path';
import * as esbuild from 'esbuild';
import { projectDir } from './utils.mjs';

const srcDir = path.resolve(projectDir, 'src');
const distDir = path.resolve(projectDir, 'dist');

await esbuild.build({
  entryPoints: [path.resolve(srcDir, 'app.tsx')],
  bundle: true,
  minify: true,
  sourcemap: true,
  target: ['chrome58', 'firefox57', 'safari11', 'edge16'],
  outfile: path.resolve(distDir, 'out.js'),
});
