#include <iostream>
#include <vector>

struct Resource {
	std::vector<uint8_t> data;
};

int main() {
	Resource resource;
	resource.data = {1, 2, 3, 4, 5};
	// Manual memory cleanup for 'data' is necessary before going out of scope
	return 0;
}

// g++ gc.cpp -o gc && ./gc
