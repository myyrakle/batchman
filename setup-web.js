const fs = require("fs");

async function main() {
  // Step 1: JS Bundle
  const sourceBundlePath = "./web/dist/bundle.js";
  const sourceBundleText = await fs.promises.readFile(sourceBundlePath, "utf8");

  const targetBundlePath = "./src/jsbundle.rs";
  const targetBundleCode = `pub const JS_BUNDLE: &str = r##"${sourceBundleText}"##;`;
  await fs.promises.writeFile(targetBundlePath, targetBundleCode);

  // Step 2: CSS Bundle
  const sourceCssPath = "./web/dist/index.css";
  const sourceCssText = await fs.promises.readFile(sourceCssPath, "utf8");
  const targetCssPath = "./src/cssbundle.rs";
  const targetCssCode = `pub const CSS_BUNDLE: &str = r##"${sourceCssText}"##;`;
  await fs.promises.writeFile(targetCssPath, targetCssCode);
}

main();
