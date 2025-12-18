#!/usr/bin/env python3
"""
Script to generate package registration updates for package_support.rs
This generates the missing fields for all packages
"""

packages = {
    # Python packages
    "django": {
        "language": "python",
        "categories": ["Web"],
        "license": "BSD-3-Clause",
        "repository": "https://github.com/django/django",
        "homepage": "https://www.djangoproject.com",
        "keywords": ["web", "framework", "orm", "mvc"]
    },
    "flask": {
        "language": "python",
        "categories": ["Web"],
        "license": "BSD-3-Clause",
        "repository": "https://github.com/pallets/flask",
        "homepage": "https://flask.palletsprojects.com",
        "keywords": ["web", "framework", "wsgi", "micro"]
    },
    "fastapi": {
        "language": "python",
        "categories": ["Web"],
        "license": "MIT",
        "repository": "https://github.com/tiangolo/fastapi",
        "homepage": "https://fastapi.tiangolo.com",
        "keywords": ["web", "api", "async", "rest"]
    },
    "pydantic": {
        "language": "python",
        "categories": ["Serialization"],
        "license": "MIT",
        "repository": "https://github.com/pydantic/pydantic",
        "homepage": "https://docs.pydantic.dev",
        "keywords": ["validation", "serialization", "schema"]
    },
    "numpy": {
        "language": "python",
        "categories": ["DataScience"],
        "license": "BSD-3-Clause",
        "repository": "https://github.com/numpy/numpy",
        "homepage": "https://numpy.org",
        "keywords": ["scientific", "computing", "arrays", "math"]
    },
    "pandas": {
        "language": "python",
        "categories": ["DataScience"],
        "license": "BSD-3-Clause",
        "repository": "https://github.com/pandas-dev/pandas",
        "homepage": "https://pandas.pydata.org",
        "keywords": ["data", "analysis", "dataframe", "statistics"]
    },
    # JavaScript packages
    "react": {
        "language": "javascript",
        "categories": ["Web", "GUI"],
        "license": "MIT",
        "repository": "https://github.com/facebook/react",
        "homepage": "https://react.dev",
        "keywords": ["ui", "components", "jsx", "frontend"]
    },
    "angular": {
        "language": "javascript",
        "categories": ["Web"],
        "license": "MIT",
        "repository": "https://github.com/angular/angular",
        "homepage": "https://angular.io",
        "keywords": ["web", "framework", "typescript", "spa"]
    },
    "vue": {
        "language": "javascript",
        "categories": ["Web"],
        "license": "MIT",
        "repository": "https://github.com/vuejs/core",
        "homepage": "https://vuejs.org",
        "keywords": ["web", "framework", "reactive", "spa"]
    },
    "nodejs": {
        "language": "javascript",
        "categories": ["Async", "Networking"],
        "license": "MIT",
        "repository": "https://github.com/nodejs/node",
        "homepage": "https://nodejs.org",
        "keywords": ["runtime", "javascript", "server", "async"]
    },
    "express": {
        "language": "javascript",
        "categories": ["Web"],
        "license": "MIT",
        "repository": "https://github.com/expressjs/express",
        "homepage": "https://expressjs.com",
        "keywords": ["web", "framework", "server", "middleware"]
    },
    "d3": {
        "language": "javascript",
        "categories": ["Utility"],
        "license": "ISC",
        "repository": "https://github.com/d3/d3",
        "homepage": "https://d3js.org",
        "keywords": ["visualization", "charts", "svg", "data"]
    },
}

# Generate Rust code for each package
for name, info in packages.items():
    categories_str = ", ".join([f"PackageCategory::{cat}" for cat in info["categories"]])
    keywords_str = ", ".join([f'"{kw}".to_string()' for kw in info["keywords"]])
    
    print(f"""
                categories: vec![{categories_str}],
                license: "{info['license']}".to_string(),
                repository: Some("{info['repository']}".to_string()),
                homepage: Some("{info['homepage']}".to_string()),
                keywords: vec![{keywords_str}],
""")
