import path from 'path';
import esbuild from 'esbuild';
import browserslist from 'browserslist';
import { esbuildPluginBrowserslist } from 'esbuild-plugin-browserslist';
import { projectDir } from './utils.mjs';

const srcDir = path.resolve(projectDir, 'src');
const distDir = path.resolve(projectDir, 'dist');

await esbuild.build({
  entryPoints: [path.resolve(srcDir, 'app.tsx')],
  bundle: true,
  minify: true,
  sourcemap: true,
  outfile: path.resolve(distDir, 'out.js'),
  plugins: [
    esbuildPluginBrowserslist(browserslist('defaults'), {
      printUnknownTargets: false,
    }),
  ],
});
