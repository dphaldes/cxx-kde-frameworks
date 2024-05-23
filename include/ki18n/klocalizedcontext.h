#pragma once

#include <KLocalizedContext>

#include <QQmlApplicationEngine>
#include <QQmlContext>
#include <QQmlEngine>

namespace rust {

namespace kf6 {
void initializeEngine(QQmlEngine &engine);

}

} // namespace rust
