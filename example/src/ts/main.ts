const readline = require("readline");
const path = require("path");

// Create an interface to read lines from stdin
const rl = readline.createInterface({
  input: process.stdin,
  output: process.stdout,
  terminal: false,
});

// Loop through each line coming from Rust's stdin
rl.on("line", () => {
  try {
    // Get the directory where this script lives
    const scriptDir = __dirname;
    const htmlPath = path.join(scriptDir, "index.html");

    // 1. Fixed key syntax ("type") and absolute path assembly
    const data = {
      responseType: "template",
      responseCode: 200,
      data: JSON.stringify({
        templatePath: htmlPath,
        arguments: {
          language: "TypeScript",
        },
      }),
    };

    // 2. Convert object to a clean single-line JSON string
    const jsonResponse = JSON.stringify(data);

    // 3. CRUCIAL: print the JSON line (console.log in Node automatically flushes)
    console.log(jsonResponse);
  } catch (e: any) {
    if (e instanceof Error) {
      console.log(`ERROR: ${e.message}`);
    } else {
      console.log(`ERROR: ${String(e)}`);
    }
  }
});
