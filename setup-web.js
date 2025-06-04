const fs = require("fs");

async function main() {
  const sourceBundlePath = "./web/dist/bundle.js";
  const sourceBundleText = await fs.promises.readFile(sourceBundlePath, "utf8");

  const targetBundlePath = "./src/jsbundle.rs";
  const targetBundleCode = `pub const JS_BUNDLE: &str = r##"${sourceBundleText}"##;`;
  await fs.promises.writeFile(targetBundlePath, targetBundleCode);
}

main();
