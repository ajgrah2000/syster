import * as vscode from 'vscode';
import * as path from 'path';
import * as fs from 'fs';
import { execSync } from 'child_process';

export interface ServerLocatorOptions {
    workspaceFolder?: vscode.WorkspaceFolder;
    outputChannel?: vscode.OutputChannel;
}

/**
 * Find the syster-lsp binary using multiple search strategies
 */
export async function findServerBinary(options: ServerLocatorOptions = {}): Promise<string> {
    const { outputChannel } = options;

    const searchLocations: Array<{ name: string; path: () => string | null }> = [
        {
            name: 'Configuration setting (syster.lsp.path)',
            path: () => vscode.workspace.getConfiguration('syster').get<string>('lsp.path') || null
        }
    ];

    for (const location of searchLocations) {
        const binaryPath = location.path();
        if (binaryPath) {
            outputChannel?.appendLine(`Checking ${location.name}: ${binaryPath}`);
            
            // Check if file exists and is executable
            if (fs.existsSync(binaryPath)) {
                try {
                    fs.accessSync(binaryPath, fs.constants.X_OK);
                    outputChannel?.appendLine(`✓ Found LSP server: ${binaryPath}`);
                    return binaryPath;
                } catch {
                    outputChannel?.appendLine(`✗ File exists but is not executable: ${binaryPath}`);
                }
            } else {
                outputChannel?.appendLine(`✗ File not found: ${binaryPath}`);
            }
        }
    }

    // No server found
    const errorMessage = [
        'Could not find syster-lsp language server binary.',
        '',
        'Please set the path in settings: "syster.lsp.path"',
        '',
        'Default location: /workspaces/syster/target/release/syster-lsp'
    ].join('\n');

    throw new Error(errorMessage);
}
