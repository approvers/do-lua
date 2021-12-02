const fs = require('fs');
const https = require('https');
const path = require('path');
const process = require('process');
const { spawnSync } = require('child_process');
const tar = require('tar');

const LUA_GZ = path.join(__dirname, 'vendor', 'lua-5.4.3.tar.gz');

(async () => {
await new Promise((resolve) => https.get("https://www.lua.org/ftp/lua-5.4.3.tar.gz", (res) => {
    const filePath = fs.createWriteStream(LUA_GZ);
    res.pipe(filePath);
    filePath.on('finish', () => {
        filePath.close();
        resolve();
    });
}));

process.chdir(path.join(__dirname, 'vendor'));

await tar.x({
    file: LUA_GZ,
});

process.chdir(path.join(__dirname, 'vendor', 'lua-5.4.3'));

const args = ['NYCFLAGS=-fPIC'];
if (process.platform === 'win32') {
    args.push('PLAT=mingw');
}
spawnSync('make', [...args, 'all', '-j4']);
})();
