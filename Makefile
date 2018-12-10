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

schema:
	diesel print-schema -o -- forum_topics forum_posts > src/plugins/forum/dao/schema.rs
	diesel print-schema -o -- vip_members > src/plugins/vip/dao/schema.rs
	diesel print-schema -o -- survey_forms survey_fields survey_responses survey_logs > src/plugins/survey/dao/schema.rs
