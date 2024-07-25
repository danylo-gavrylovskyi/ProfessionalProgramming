#pragma once

#include <functional>
#include <vector>
#include <mutex>

#include "Messages.hpp"

class MessageDispatcher {
public:
    using GreenMessageHandler = std::function<void(const GreenMessage&)>;
    using BlueMessageHandler = std::function<void(const BlueMessage&)>;
    using OrangeMessageHandler = std::function<void(const OrangeMessage&)>;

    void subscribeToGreenMessage(const GreenMessageHandler& handler);
    void subscribeToBlueMessage(const BlueMessageHandler& handler);
    void subscribeToOrangeMessage(const OrangeMessageHandler& handler);

    void publishGreenMessage(const GreenMessage& message);
    void publishBlueMessage(const BlueMessage& message);
    void publishOrangeMessage(const OrangeMessage& message);

private:
    std::vector<GreenMessageHandler> greenMessageHandlers;
    std::vector<BlueMessageHandler> blueMessageHandlers;
    std::vector<OrangeMessageHandler> orangeMessageHandlers;
    std::mutex mtx;
};
