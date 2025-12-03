import * as vscode from 'vscode';
import { exec } from 'child_process';
import { promisify } from 'util';
import * as path from 'path';

const execAsync = promisify(exec);

interface UpdateCheckResult {
    has_updates: boolean;
    commits_behind: number;
    changed_files: string[];
    project_name: string;
}

export function activate(context: vscode.ExtensionContext) {
    console.log('Cursor Rules Sync extension is now active');

    // Find shared repo scripts path
    const workspaceFolders = vscode.workspace.workspaceFolders;
    if (!workspaceFolders || workspaceFolders.length === 0) {
        return;
    }

    const workspaceRoot = workspaceFolders[0].uri.fsPath;
    const scriptsPath = path.join(workspaceRoot, '___shared', '.cursor-private-git', 'scripts');

    // Check for updates on startup
    checkForUpdates(scriptsPath);

    // Register commands
    const syncCommand = vscode.commands.registerCommand('cursorRulesSync.sync', async () => {
        await syncRules(scriptsPath);
    });

    const checkCommand = vscode.commands.registerCommand('cursorRulesSync.check', async () => {
        await checkForUpdates(scriptsPath);
    });

    // Check for updates when Git panel refreshes
    const gitExtension = vscode.extensions.getExtension('vscode.git');
    if (gitExtension) {
        // Listen for Git repository changes
        vscode.workspace.onDidChangeWorkspaceFolders(async () => {
            await checkForUpdates(scriptsPath);
        });
    }

    context.subscriptions.push(syncCommand, checkCommand);
}

async function checkForUpdates(scriptsPath: string): Promise<void> {
    const checkScript = path.join(scriptsPath, 'check-cursor-rules-updates');
    
    try {
        const { stdout } = await execAsync(`python "${checkScript}" --json`, {
            cwd: path.dirname(scriptsPath)
        });

        const result: UpdateCheckResult = JSON.parse(stdout);
        
        if (result.has_updates) {
            const message = `Cursor Rules: ${result.commits_behind} commit(s) behind remote. ${result.changed_files.length} file(s) changed.`;
            
            vscode.window.showInformationMessage(
                message,
                'Sync Now'
            ).then(selection => {
                if (selection === 'Sync Now') {
                    syncRules(scriptsPath);
                }
            });
        }
    } catch (error: any) {
        // Script might not exist or error occurred
        // Silently fail - this is optional functionality
        if (error.code !== 'ENOENT') {
            console.error('Error checking for updates:', error);
        }
    }
}

async function syncRules(scriptsPath: string): Promise<void> {
    const syncScript = path.join(scriptsPath, 'sync-cursor-rules');
    
    const terminal = vscode.window.createTerminal('Cursor Rules Sync');
    terminal.show();
    terminal.sendText(`python "${syncScript}"`);
    
    vscode.window.showInformationMessage('Cursor Rules sync started in terminal');
}

export function deactivate() {}




