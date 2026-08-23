const { execFile } = require("child_process");
const vscode = require("vscode");

const activate = (_) => {
  const outputChannel = vscode.window.createOutputChannel("nixfmt");

  vscode.languages.registerDocumentFormattingEditProvider("nix", {
    provideDocumentFormattingEdits(document, _) {
      const config = {
        nixfmt: vscode.workspace.getConfiguration("nixfmt"),
      };

      return new Promise((resolve, reject) => {
        try {
          outputChannel.appendLine(
            `Running nixfmt with settings: ${JSON.stringify(config)}`
          );

          const process = execFile(
            config.nixfmt.program,
            [],
            {},
            (error, stdout, stderr) => {
              if (error) {
                outputChannel.appendLine(`error: ${error}`);
                outputChannel.appendLine(`stderr: ${stderr}`);
                vscode.window.showErrorMessage(
                  `While executing nixfmt with settings: ` +
                    `${JSON.stringify(config)}, ` +
                    `${error}`
                );
                reject(error);
              }

              const documentRange = new vscode.Range(
                document.lineAt(0).range.start,
                document.lineAt(
                  document.lineCount - 1
                ).rangeIncludingLineBreak.end
              );

              resolve([new vscode.TextEdit(documentRange, stdout)]);
            }
          );

          const documentText = document.getText();

          outputChannel.appendLine(
            `Feeding ${documentText.length} of input to stdin`
          );

          process.stdin.write(documentText);
          process.stdin.end();
        } catch (error) {
          vscode.window.showErrorMessage(
            `While executing nixfmt with settings: ` +
              `${JSON.stringify(config)} ` +
              `${error}`
          );
          reject(error);
        }
      });
    },
  });
};

const deactivate = () => {};

module.exports = {
  activate,
  deactivate,
};
