{
  "targets": [
    {
      "target_name": "lua-js",
      "sources": [
        "src/entry.cpp",
        "src/dofile.cpp"
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
