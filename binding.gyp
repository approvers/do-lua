{
    "targets": [
        {
            "target_name": "lua-js",
            "sources": [
                "src/convert.cpp",
                "src/entry.cpp",
                "src/dofile.cpp",
                "src/dostring.cpp",
                "src/lua_program.cpp",
            ],
            "include_dirs": [
                "<!(node -e \"require('nan')\")",
                "vendor/lua-5.4.3/src/",
            ],
            "libraries": ["-Lvendor/lua-5.4.3/src", "-llua"],
        }
    ]
}
