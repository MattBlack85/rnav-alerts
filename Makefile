.PHONY = build-doc

build-doc:
	@cargo doc

.PHONY = bump-patch-version

bump-patch-version:
	@zsh scripts/bump.sh patch

.PHONY = bump-minor-version

bump-minor-version:
	@scripts/bump.sh minor

.PHONY = bump-major-version

bump-major-version:
	@scripts/bump.sh major

.PHONY = doc

doc:
	@cargo doc --open
