MMARK :=mmark
TWO := ""
TXT := $(patsubst %.md,%.txt,$(wildcard *.md))
HTML := $(patsubst %.md,%.html,$(wildcard *.md))
XML := $(patsubst %.md,%.xml,$(wildcard *.md))

txt: $(TXT)
html: $(HTML)

%.txt: %.md
	if [ -z $(TWO) ]; then \
	    $(MMARK) $< > $(basename $<).xml; \
	    xml2rfc --text --v3 $(basename $<).xml && rm $(basename $<).xml; \
	else \
	    $(MMARK) -2 $< > $(basename $<).xml;  \
	    xml2rfc --text $(basename $<).xml && rm $(basename $<).xml; \
	fi

%.html: %.md
	if [ -z $(TWO) ]; then \
	    $(MMARK) $< > $(basename $<).xml; \
		xml2rfc --html --v3 $(basename $<).xml && rm $(basename $<).xml; \
	else \
	    $(MMARK) -2 $< > $(basename $<).xml; \
		xml2rfc --html $(basename $<).xml && rm $(basename $<).xml; \
	fi

xml: $(XML)

%.xml: %.md
	if [ -z $(TWO) ]; then \
	    $(MMARK) $< > $(basename $<).xml; \
	else \
	    $(MMARK) -2 $< > $(basename $<).xml; \
	fi

.PHONY: clean
clean:
	rm -fv *.txt *.xml *.html

html_body: $(HTML)
	$(eval TMPFILE := $(shell mktemp))
	@for h in $(HTML); do \
		hxnormalize -xe "$$h" | hxselect -cis '\n' 'body' | hxremove -i 'script' | hxremove 'address' | hxremove '#external-metadata' | node -e 'process.stdin.resume(); process.stdin.setEncoding("utf8"); process.stdin.on("data", d => process.stdout.write(d.replace(/(&nbsp;|<a([^>]+\/)>)/ig, "")));' > $(TMPFILE); \
		mv $(TMPFILE) "$$h" ; \
		echo html bodied "$$h" ;\
	done
