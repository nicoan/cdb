prefix=/usr/local
PACKAGE_NAME=$(shell cat Cargo.toml | grep name | awk '{print $$3}' | sed -e 's/"//g')
PACKAGE_VERSION=$(shell cat Cargo.toml | grep version | awk '{print $$3}' | sed -e 's/"//g')
PACKAGE_COMPLETE_NAME=$(PACKAGE_NAME)_$(PACKAGE_VERSION)
UID=$(shell id -u)
GID=$(shell id -g)
CURRENT_UID=$(UID):$(GID)

all:
	cargo build --release
	strip ./target/release/$(PACKAGE_NAME)

# Basic files to copy
install-base:
	install -D $$(pwd)/target/release/cdb $(DESTDIR)$(prefix)/libexec/cdb/cdb
	install -m 644 $$(pwd)/src/scripts/cdb.sh $(DESTDIR)/etc/profile.d/
	sed -i '1iexport CDB_MANAGER_PATH=$(prefix)\/libexec\/cdb/cdb\n' $(DESTDIR)/etc/profile.d/cdb.sh
	install -D $$(pwd)/doc/cdb.1 $(DESTDIR)$(prefix)/share/man/man1/cdb.1

# Install compiling from source
install: install-base
	install $$(pwd)/src/scripts/cdb.bash-completion /usr/share/bash-completion/completions/cdb
	src/scripts/postinst

# Uninstall from source
uninstall:
	rm -r $(prefix)/lib/cdb
	rm /etc/profile.d/cdb.sh
	rm $(prefix)/share/man/man1/cdb.1
	rm /usr/share/bash-completion/completions/cdb
	src/scripts/postrm

clean:
	cargo clean

##
# For packaging
##

dpkg: dpkg-clean dpkg-deb dpkg-rpm

dpkg-clean:
	rm -rf packages

##
# Debian

# Install for deb packaging
install-deb: install-base

PACKAGES_DIR_DEB=./packages/deb

$(PACKAGES_DIR_DEB):
	$(MAKE) dpkg-deb

dpkg-deb:
	# Clean packages directory
	mkdir -p $(PACKAGES_DIR_DEB)
	# Create .orig tar
	tar -czvf $(PACKAGES_DIR_DEB)/$(PACKAGE_COMPLETE_NAME).orig.tar.gz \
		--exclude "release" \
		--exclude ".git" \
		--exclude ".gitignore" \
		--exclude "debian" \
		--exclude "packages" \
		--exclude "target" \
		--transform s/./$(PACKAGE_COMPLETE_NAME)/ \
		.
	# Uncompress .orig un ./packages
	cd $(PACKAGES_DIR_DEB) && tar xf $(PACKAGE_COMPLETE_NAME).orig.tar.gz
	# Copy debian directory to create packages
	cp -r ./debian $(PACKAGES_DIR_DEB)/$(PACKAGE_COMPLETE_NAME)
	# Build deb package from source
	cd $(PACKAGES_DIR_DEB)/$(PACKAGE_COMPLETE_NAME) && debuild -us -uc
	# Cleanup extracted tar file
	rm -r $(PACKAGES_DIR_DEB)/$(PACKAGE_COMPLETE_NAME)

##
# RPM

PACKAGES_DIR_RPM=./packages/rpm
dpkg-rpm: $(PACKAGES_DIR_DEB)
	# Make rpm dir
	mkdir -p $(PACKAGES_DIR_RPM)
	# Build rpm package with alien
	cd $(PACKAGES_DIR_RPM) && ls ../deb/$(PACKAGE_COMPLETE_NAME)*.deb | xargs fakeroot alien -r --scripts

# We use a docker image to generate the packages...
dpkg-docker:
	docker build --build-arg USER=$(USER) --build-arg UID=$(UID) --build-arg GID=$(GID) -t cdb_debian_dpkg .
	docker run -it --rm --name cdb_dpkg_builder -v $$(pwd):/home/$(USER)/dpkg/src -u $(CURRENT_UID) cdb_debian_dpkg make dpkg
	# We may want to create the instance for debugging or smotheing, we do that this way :)
	# docker create -it --name cdb_dpkg_builder -v $$(pwd):/home/$(USER)/dpkg/src -u $(CURRENT_UID) cdb_debian_dpkg

.PHONY: install uninstall clean all install-deb install-base dpkg-deb dpkg-rpm dpkg-clean dpkg
