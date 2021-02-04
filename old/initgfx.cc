# include <luma/main.hh>
# include <vulkan/vulkan.h>
# include <wayland-client.h>
# include <xcb/xcb.h>
void luma::app_t::initgfx() {
	this->gfxisinit = true;
	// Determine if X should be used or if Wayland is da way.
	this->setdispsrvproto();
	if(this->dispsrvproto == this->dispsrvproto_t::x) {
		this->dbgmsg("Creating X connection... ");
		this->xconn = xcb_connect(nullptr,nullptr);
		this->xscrn = xcb_setup_roots_iterator(xcb_get_setup(this->xconn)).data;
		this->xwin  = xcb_generate_id(this->xconn);
		this->dbgmsg("O.K.\n");
	}
	else {
		this->dbgmsg("Creating Wayland connection... ");
		this->wldisp = wl_display_connect(nullptr);
		if(this->wldisp == nullptr) {
			this->dbgmsg("Error\n");
		}
		else {
			this->dbgmsg("O.K.\n");
		}
	}
	// Set data required by Vulkan.
	this->vkappinf.sType                 = VK_STRUCTURE_TYPE_APPLICATION_INFO;
	this->vkappinf.pApplicationName      = "Luma Standard Library";
	this->vkappinf.pEngineName           = "Luma Standard Library";
	this->vkinstcrtinf.sType             = VK_STRUCTURE_TYPE_INSTANCE_CREATE_INFO;
	this->vkinstcrtinf.pApplicationInfo  = &this->vkappinf;
	this->vkinstcrtinf.enabledLayerCount = 0x0;
	this->dbgmsg("Creating Vulkan instance... ");
	this->vkreslt = vkCreateInstance(&this->vkinstcrtinf,nullptr,&this->vkinst);
	if(this->vkreslt == VK_SUCCESS) {
		this->dbgmsg("O.K.\n");
	}
	else {
		this->dbgmsg("Error\n");
	}
	int extcount = 0x0;
	this->vkexts = std::vector<VkExtensionProperties>(extcount);
	::vkEnumerateInstanceExtensionProperties(nullptr,&extcount,this->vkexts.data());
	//this->dbgmsg(std::basic_string<char_t> {std::to_string(extcount) + " Vulkan extensions supported.\n"}.c_str());
	this->dbgmsg("The following Vulkan extensions are supported:\n");
	for(auto const & ext : this->vkexts) {
		this->dbgmsg(ext.extensionName);
	}
	int devcount = 0x0;
	::vkEnumeratePhysicalDevices(this->vkinst,&devcount,this->vkphysdevs.data());
	//this->dbgmsg(std::basic_string<char> {std::to_string(devcount) + " Vulkan-compatible device(s) found.\n"}.c_str());
}
