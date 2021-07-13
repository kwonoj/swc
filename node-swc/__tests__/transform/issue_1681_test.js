const swc = require('../../..');

it('should work with parser.syntax', () => {
  const input =
    `console.log(String.raw\`a
b
c
\`);
console.log(String.raw\`a\\nb\\nc\\n\`);`
  const output = swc.transformSync(input, {
    env: {
      targets: {
        chrome: 88
      },
      //debug: true
    },
  }).code.trim();

  expect(input).toEqual(output);
});
