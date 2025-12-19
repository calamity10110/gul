#!/usr/bin/env python3
"""Generate changelog from git commits."""

import subprocess
import os
import sys
from datetime import datetime


def get_git_log(since: str = None) -> list:
    """Get git commits since a given date or tag."""
    cmd = ['git', 'log', '--pretty=format:%H|%s|%an|%ad', '--date=short']
    if since:
        cmd.extend(['--since', since])
    
    try:
        result = subprocess.run(cmd, capture_output=True, text=True, check=True)
        commits = []
        for line in result.stdout.strip().split('\n'):
            if line:
                parts = line.split('|')
                if len(parts) >= 4:
                    commits.append({
                        'hash': parts[0][:8],
                        'message': parts[1],
                        'author': parts[2],
                        'date': parts[3]
                    })
        return commits
    except subprocess.CalledProcessError:
        return []


def categorize_commits(commits: list) -> dict:
    """Categorize commits by type."""
    categories = {
        'Features': [],
        'Bug Fixes': [],
        'Documentation': [],
        'Refactoring': [],
        'Other': []
    }
    
    for commit in commits:
        msg = commit['message'].lower()
        if msg.startswith('feat') or 'add' in msg or 'new' in msg:
            categories['Features'].append(commit)
        elif msg.startswith('fix') or 'bug' in msg:
            categories['Bug Fixes'].append(commit)
        elif msg.startswith('doc') or 'readme' in msg:
            categories['Documentation'].append(commit)
        elif msg.startswith('refactor') or 'clean' in msg:
            categories['Refactoring'].append(commit)
        else:
            categories['Other'].append(commit)
    
    return categories


def generate_changelog(commits: list) -> str:
    """Generate changelog content."""
    categories = categorize_commits(commits)
    today = datetime.now().strftime('%Y-%m-%d')
    
    changelog = f"# Changelog\n\n## [{today}]\n\n"
    
    for category, items in categories.items():
        if items:
            changelog += f"### {category}\n\n"
            for commit in items:
                changelog += f"- {commit['message']} ({commit['hash']})\n"
            changelog += "\n"
    
    return changelog


def main():
    """Main entry point."""
    commits = get_git_log(since="1 week ago")
    
    if not commits:
        print("No recent commits found")
        sys.exit(0)
    
    changelog = generate_changelog(commits)
    
    output_file = os.environ.get('CHANGELOG_FILE', 'CHANGES.md')
    
    # Append to existing or create new
    existing = ""
    if os.path.exists(output_file):
        with open(output_file, 'r') as f:
            existing = f.read()
    
    with open(output_file, 'w') as f:
        # Only write header if new
        if not existing.startswith("# Changelog"):
            f.write(changelog)
        else:
            f.write(existing)
    
    print(f"Changelog updated: {output_file}")


if __name__ == "__main__":
    main()
