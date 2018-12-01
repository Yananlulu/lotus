dist=dist

build: api ds
	cd $(dist) && tar cfJ ../$(dist).tar.xz *

api:
	GIT_HEAD=`git rev-parse --short HEAD` BUILD_TIME=`date -R` cargo build --release
	strip -s target/release/lotus
	mkdir -p $(dist)/public $(dist)/tmp
	-cp -r target/release/lotus log4rs.yml package.json package-lock.json LICENSE README.md $(dist)/
	-cp -r third/ueditor/dist $(dist)/public/ueditor

ds:
	cd dashboard && umi build
	-cp -r dashboard/dist $(dist)/dashboard

clean:
	cargo clean
	-rm -r $(dist) $(dist).tar.xz dashboard/dist

init:
	git submodule update
	npm install
	cd dashboard && npm install
	cd third/ueditor && npm install && grunt --encode=utf8 --server=jsp
