const rust = import("./parser");

rust.then(
    parser => console.log(parser.root("<!-- wp:foo /-->"))
);
