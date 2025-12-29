"""
GUL SemVer
Semantic Versioning utilities.

Status: ✅ Implemented
Priority: Medium
"""

from typing import Optional, Tuple, Union
import re

__version__ = "0.1.0"
__all__ = ['Version', 'parse', 'bump_major', 'bump_minor', 'bump_patch']

class Version:
    """Semantic Version"""
    
    def __init__(
        self,
        major: int,
        minor: int,
        patch: int,
        prerelease: Optional[str] = None,
        build: Optional[str] = None
    ):
        self.major = major
        self.minor = minor
        self.patch = patch
        self.prerelease = prerelease
        self.build = build
    
    def __str__(self) -> str:
        ver = f"{self.major}.{self.minor}.{self.patch}"
        if self.prerelease:
            ver += f"-{self.prerelease}"
        if self.build:
            ver += f"+{self.build}"
        return ver
    
    def __repr__(self) -> str:
        return f"Version('{str(self)}')"
    
    def __eq__(self, other) -> bool:
        if isinstance(other, str):
            other = parse(other)
        return (
            self.major == other.major and
            self.minor == other.minor and
            self.patch == other.patch and
            self.prerelease == other.prerelease
        )
    
    def __lt__(self, other) -> bool:
        if isinstance(other, str):
            other = parse(other)
        
        if self.major != other.major:
            return self.major < other.major
        if self.minor != other.minor:
            return self.minor < other.minor
        if self.patch != other.patch:
            return self.patch < other.patch
        
        # Simplified prerelease handling (existence check)
        # Real semver logic for prerelease is more complex
        if self.prerelease and not other.prerelease:
            return True
        if not self.prerelease and other.prerelease:
            return False
            
        return False

    def next_major(self) -> 'Version':
        return Version(self.major + 1, 0, 0)
    
    def next_minor(self) -> 'Version':
        return Version(self.major, self.minor + 1, 0)
    
    def next_patch(self) -> 'Version':
        return Version(self.major, self.minor, self.patch + 1)

def parse(version_str: str) -> Version:
    """Parse version string"""
    # Simplified Regex for SemVer 2.0.0
    pattern = r'^(?P<major>0|[1-9]\d*)\.(?P<minor>0|[1-9]\d*)\.(?P<patch>0|[1-9]\d*)(?:-(?P<prerelease>(?:0|[1-9]\d*|\d*[a-zA-Z-][0-9a-zA-Z-]*)(?:\.(?:0|[1-9]\d*|\d*[a-zA-Z-][0-9a-zA-Z-]*))*))?(?:\+(?P<build>[0-9a-zA-Z-]+(?:\.[0-9a-zA-Z-]+)*))?$'
    
    match = re.match(pattern, version_str)
    if not match:
        raise ValueError(f"Invalid version string: {version_str}")
    
    parts = match.groupdict()
    
    return Version(
        major=int(parts['major']),
        minor=int(parts['minor']),
        patch=int(parts['patch']),
        prerelease=parts['prerelease'],
        build=parts['build']
    )

def bump_major(version: str) -> str:
    return str(parse(version).next_major())

def bump_minor(version: str) -> str:
    return str(parse(version).next_minor())

def bump_patch(version: str) -> str:
    return str(parse(version).next_patch())
