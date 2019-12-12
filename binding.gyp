{
  "targets": [
    {
      "target_name": "lua-js",
      "sources": [
        "src/entry.cpp",
        "src/do_file.cpp"
      ],
      "include_dirs": [
        "<!(node -e \"require('nan')\")",
        "/usr/local/include",
        "/usr/include"
      ],
      "libraries": [
        "-llua"
      ]
    }
  ]
}
