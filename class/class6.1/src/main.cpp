#include <print>
#include <thread>
#include <random>

#include "MessageDispatcher.hpp"

void generateMessages(MessageDispatcher& dispatcher, int threadId) {
    std::random_device rd;
    std::mt19937 gen(rd());
    std::uniform_int_distribution<> dist(1, 3);
    std::uniform_int_distribution<> intDist(1, 100);
    std::uniform_real_distribution<> doubleDist(1.0, 100.0);
    std::uniform_int_distribution<> charDist(65, 90);

    for (int i = 0; i < 10; ++i) {
        int messageType = dist(gen);
        if (messageType == 1) {
            GreenMessage msg{"GreenMessage from thread " + std::to_string(threadId), intDist(gen)};
            dispatcher.publishGreenMessage(msg);
        } else if (messageType == 2) {
            BlueMessage msg{doubleDist(gen), doubleDist(gen)};
            dispatcher.publishBlueMessage(msg);
        } else if (messageType == 3) {
            OrangeMessage msg{
                std::string(1, charDist(gen)),
                std::string(1, charDist(gen)),
                intDist(gen),
                doubleDist(gen)
            };
            dispatcher.publishOrangeMessage(msg);
        }
        std::this_thread::sleep_for(std::chrono::milliseconds(100));
    }
}

int main() {
    MessageDispatcher dispatcher;

    dispatcher.subscribeToGreenMessage([](const GreenMessage& msg) {
        std::println("Received GreenMessage: {}, counter: {}\n", msg.text, msg.counter);
    });

    dispatcher.subscribeToBlueMessage([](const BlueMessage& msg) {
        std::println("Received BlueMessage: {}, {}\n", msg.value1, msg.value2);
    });

    dispatcher.subscribeToOrangeMessage([](const OrangeMessage& msg) {
        std::println("Received OrangeMessage: {}, {}, {}, {}\n", msg.text1, msg.text2, msg.integer, msg.value);
    });

    std::thread t1(generateMessages, std::ref(dispatcher), 1);
    std::thread t2(generateMessages, std::ref(dispatcher), 2);

    t1.join();
    t2.join();

    return 0;
}
