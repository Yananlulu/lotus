dist=dist

build: api www
	cd $(dist) && tar cfJ ../$(dist).tar.xz *

api:
	GIT_HEAD=`git rev-parse --short HEAD` BUILD_TIME=`date -R` cargo build --release
	strip -s target/release/lotus
	mkdir -p $(dist)/public $(dist)/tmp
	-cp -r target/release/lotus locales tools templates themes log4rs.yml package.json package-lock.json LICENSE README.md $(dist)/
	-cp -r third/ueditor/dist $(dist)/public/ueditor

www:
	cd dashboard && npm run build
	-cp -r dashboard/build $(dist)/dashboard

clean:
	cargo clean
	-rm -r $(dist) $(dist).tar.xz dashboard/build

init:
	git submodule update
	npm install
	cd dashboard && npm install
	cd third/ueditor && npm install && grunt --encode=utf8 --server=jsp
