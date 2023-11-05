#include <iostream>
#include <thread>
#include <vector>

void process_item(int item) {
	std::cout << "Processed: " << item * 2 << std::endl;
}

int main() {
	std::vector<int> data = {1, 2, 3, 4, 5};
	std::vector<std::thread> threads;

	for (const auto &item : data) {
		threads.push_back(std::thread(process_item, item));
	}

	for (auto &thread : threads) {
		thread.join();
	}

	return 0;
}

// Processed: Processed: 26

// Processed: 4
// Processed: 8
// Processed: 10
