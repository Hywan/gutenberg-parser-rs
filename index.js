const rust = import("./parser");

rust.then(
    parser => {
        const data = ["<!-- wp:foo /-->", "hello"];

        for (datum of data) {
            const output = parser.root(datum);

            console.log(`\`${datum}\`: ${output}`);
        }
    }
);
