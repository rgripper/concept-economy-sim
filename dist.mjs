import fs from 'fs/promises';
import { constants } from 'fs';
import path from 'path';

if (await fs.stat('dist', constants.O_DIRECTORY)) {
    await fs.rm('dist', { recursive: true })
}

await fs.mkdir('dist');

await fs.copyFile('index.html', 'dist/index.html');
await copyDir('pkg', 'dist/pkg');

async function copyDir(src, dest) {
    await fs.mkdir(dest, { recursive: true });
    let entries = await fs.readdir(src, { withFileTypes: true });

    for (let entry of entries) {
        let srcPath = path.join(src, entry.name);
        let destPath = path.join(dest, entry.name);

        entry.isDirectory() ?
            await copyDir(srcPath, destPath) :
            await fs.copyFile(srcPath, destPath);
    }
}