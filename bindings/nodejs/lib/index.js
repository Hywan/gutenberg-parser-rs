var parser = require('../native');

var output = parser.root("<!-- wp:foo {\"abc\": \"xyz\"} --><!-- wp:bar /--><!-- /wp:foo -->");

console.log(output);
console.log(JSON.stringify(output));
