#include "MessageDispatcher.hpp"

void MessageDispatcher::subscribeToGreenMessage(const GreenMessageHandler& handler) {
    std::lock_guard<std::mutex> lock(mtx);
    greenMessageHandlers.push_back(handler);
}

void MessageDispatcher::subscribeToBlueMessage(const BlueMessageHandler& handler) {
    std::lock_guard<std::mutex> lock(mtx);
    blueMessageHandlers.push_back(handler);
}

void MessageDispatcher::subscribeToOrangeMessage(const OrangeMessageHandler& handler) {
    std::lock_guard<std::mutex> lock(mtx);
    orangeMessageHandlers.push_back(handler);
}

void MessageDispatcher::publishGreenMessage(const GreenMessage& message) {
    std::lock_guard<std::mutex> lock(mtx);
    for (const auto& handler : greenMessageHandlers) {
        handler(message);
    }
}

void MessageDispatcher::publishBlueMessage(const BlueMessage& message) {
    std::lock_guard<std::mutex> lock(mtx);
    for (const auto& handler : blueMessageHandlers) {
        handler(message);
    }
}

void MessageDispatcher::publishOrangeMessage(const OrangeMessage& message) {
    std::lock_guard<std::mutex> lock(mtx);
    for (const auto& handler : orangeMessageHandlers) {
        handler(message);
    }
}
