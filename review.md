# review full project rename the language GLOB,

# add more function:

## allow to do import scripts/packages, define definitions, add async/non-async functions, in block:

'''
import python : numpy.
import python : matplotlib.
import rust : tokio,
import blob_pak_1.
import blob_pak_2.
'''
is the same as
'''
import [
python : (numpy, matplotlib),
rust : tokio,
blob_pak_1,
blob_pak_2].
'''
or
'''
import python : {numpy, matplotlib}.
import rust : tokio.
import (blob_pak_1, blob_pak_2).
'''

## [], {}, () work the same, as long as they match.

## ? is same as mut in rust, allow to change the value.

## global variables are manage by async-func, static variables are manage by func(non-async) and async-func.

## add @asy, @fnc, @cs,

# test, debug all features in documentation.

# add information to syntax.md and structure.md to fully explain the language syntax & structure, coding patterns, best practice, examples, add annotation to codebase, aim for beginner.

# review and finish update documentation: compiler, intergration, plan, progress, test_verification_report, tui, webui, syntax, structure(project and language) readme, rebrand_summary.
