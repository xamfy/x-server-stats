#### Branch naming convention

Following are the branch naming convention for the repository.

  * `main` - The main branch where the source code of HEAD always reflects a production-ready state.
  * `develop` - The main branch where the source code of HEAD always reflects a state with the latest delivered development changes for the next release.
  * `feature/*` - Branch from `develop` and merge back into `develop`. Branch naming convention is `feature/<issue-number>-<issue-title>`. For example, `feature/1234-add-branch-naming-convention`.
  * `release/*` - Branch from `develop` and merge back into `develop` and `main`. Branch naming convention is `release/<version-number>`. For example, `release/1.0.0`.
  * `hotfix/*` - Branch from `main` and merge back into `develop` and `main`. Branch naming convention is `hotfix/<issue-number>-<issue-title>`. For example, `hotfix/1234-fix-branch-naming-convention`.
