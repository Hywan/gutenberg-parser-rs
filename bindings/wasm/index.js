const rust = import("./parser");

rust.then(
    parser => {
        const data = [`<!-- wp:foo {"bar": "qux"} /--><!-- wp:bar /-->`, `hello`];

        for (let datum of data) {
            parser.root(datum);
        }
    }
);
