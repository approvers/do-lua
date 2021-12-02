{
    "defines": ["V8_DEPRECATION_WARNINGS=1"],
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
                "<(module_root_dir)/vendor/lua-5.4.3/src/",
            ],
            "libraries": [
                "-llua",
                "-L<(module_root_dir)/vendor/lua-5.4.3/src/",
            ],
            "conditions": [
                [
                    'OS=="win"',
                    {
                        "libraries": [
                            "<(module_root_dir)/vendor/lua-5.4.3/src/lua",
                        ],
                    },
                ]
            ],
        }
    ],
}
