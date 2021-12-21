const { readFileSync } = require("fs");
const { StringDecoder } = require("string_decoder");

// let's use the synchronous variant for now.
const contents = readFileSync("/etc/hosts", { encoding: "utf-8"});

console.log(contents);
