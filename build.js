const { chdir, platform } = require('process');
const fs = require('fs');
const https = require('https');
const path = require('path');
const { spawnSync } = require('child_process');
const tar = require('tar');

const LUA_GZ = path.join(__dirname, 'vendor', 'lua-5.4.3.tar.gz');
const LUA_PATH = path.join(__dirname, 'vendor', 'lua-5.4.3');

(async () => {
await new Promise((resolve) => https.get("https://www.lua.org/ftp/lua-5.4.3.tar.gz", (res) => {
    const filePath = fs.createWriteStream(LUA_GZ);
    res.pipe(filePath);
    filePath.on('finish', () => {
        filePath.close();
        resolve();
    });
}));

chdir(path.join(__dirname, 'vendor'));

await tar.x({
    file: LUA_GZ,
});

chdir(LUA_PATH);

const args = ['MYCFLAGS=-fPIC'];
console.log(`Current platform: ${platform}`);
if (platform === 'win32') {
    args.push('PLAT=mingw');
}
spawnSync('make', [...args, 'all', '-j4']);

if (platform === 'win32') {
    chdir(path.join(LUA_PATH, 'src'));
    spawnSync('lib', ['/OUT:lua.lib', '*.obj']);
}
})();
