import url from 'url';
import path from 'path';

const scriptDir = url.fileURLToPath(new URL('.', import.meta.url));

export const projectDir = path.dirname(scriptDir);
