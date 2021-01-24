# include <luma/main.hh>
# include <vulkan/vulkan.h>
# include <wayland-client.h>
# include <xcb/xcb.h>
void luma::initgfx() {
	luma::dat.gfxisinit = true;
	// Determine if X should be used or if Wayland is da way.
	luma::setdispsrvproto();
	if(luma::dat.dispsrvproto == luma::dispsrvproto_t::x) {
		luma::dbgmsg("Creating X connection... ");
		luma::dat.xconn = xcb_connect(nullptr,nullptr);
		luma::dat.xscrn = xcb_setup_roots_iterator(xcb_get_setup(luma::dat.xconn)).data;
		luma::dat.xwin  = xcb_generate_id(luma::dat.xconn);
		luma::dbgmsg("O.K.\n");
	}
	else {
		luma::dbgmsg("Creating Wayland connection... ");
		luma::dat.wldisp = wl_display_connect(nullptr);
		if(luma::dat.wldisp == nullptr) {
			luma::dbgmsg("Error\n");
		}
		else {
			luma::dbgmsg("O.K.\n");
		}
	}
	// Set data required by Vulkan.
	luma::dat.vkappinf.sType                 = VK_STRUCTURE_TYPE_APPLICATION_INFO;
	luma::dat.vkappinf.pApplicationName      = "Luma Standard Library";
	luma::dat.vkappinf.pEngineName           = "Luma Standard Library";
	luma::dat.vkinstcrtinf.sType             = VK_STRUCTURE_TYPE_INSTANCE_CREATE_INFO;
	luma::dat.vkinstcrtinf.pApplicationInfo  = &luma::dat.vkappinf;
	luma::dat.vkinstcrtinf.enabledLayerCount = 0x0;
	luma::dbgmsg("Creating Vulkan instance... ");
	luma::dat.vkreslt = vkCreateInstance(&luma::dat.vkinstcrtinf,nullptr,&luma::dat.vkinst);
	if(luma::dat.vkreslt != VK_SUCCESS) {
		luma::dbgmsg("Error\n");
	}
	else {
		luma::dbgmsg("O.K.\n");
	}
	std::uint32_t extcount = 0x0;
	luma::dat.vkexts = std::vector<VkExtensionProperties>(extcount);
	::vkEnumerateInstanceExtensionProperties(nullptr,&extcount,luma::dat.vkexts.data());
	luma::dbgmsg(std::basic_string<char> {std::to_string(extcount) + " Vulkan extensions supported.\n"}.c_str());
	luma::dbgmsg("The following Vulkan extensions are supported:\n");
	for(auto const & ext : luma::dat.vkexts) {
		luma::dbgmsg(ext.extensionName);
	}
	std::uint32_t devcount = 0x0;
	::vkEnumeratePhysicalDevices(luma::dat.vkinst,&devcount,luma::dat.vkphysdevs.data());
	luma::dbgmsg(std::basic_string<char> {std::to_string(devcount) + " Vulkan-compatible device(s) found.\n"}.c_str());
}
