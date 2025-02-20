RUST_DIR := $(dir $(lastword $(MAKEFILE_LIST)))
PARTY_GRAPHICS := $(RUST_DIR)/graphics/party_menu_full
CPPFLAGS += -iquote $(RUST_DIR)/include

$(PARTY_GRAPHICS)/tera/tera_types.gbapal:\
		$(PARTY_GRAPHICS)/tera/tera_types_1.gbapal\
		$(PARTY_GRAPHICS)/tera/tera_types_2.gbapal\
		$(PARTY_GRAPHICS)/tera/tera_types_3.gbapal
	@cat $^ > $@

$(PARTY_GRAPHICS)/tera/tera_types.4bpp: $(types:%=$(PARTY_GRAPHICS)/tera/%.4bpp)
	@cat $^ > $@

RUST_INC_FILES :=\
	$(PARTY_GRAPHICS)/bg.bin.lz\
	$(PARTY_GRAPHICS)/tiles.gbapal.lz\
	$(PARTY_GRAPHICS)/tiles.4bpp.lz\
	$(PARTY_GRAPHICS)/mon_bg.bin.lz\
	$(PARTY_GRAPHICS)/hp.plain.bin.lz\
	$(PARTY_GRAPHICS)/status_icons.4bpp.lz\
	$(PARTY_GRAPHICS)/status_icons.gbapal.lz\
	$(PARTY_GRAPHICS)/tera/tera_types.4bpp.lz\
	$(PARTY_GRAPHICS)/tera/tera_types.gbapal.lz

librust: $(RUST_INC_FILES)
	cd $(RUST_DIR) && cargo build --release