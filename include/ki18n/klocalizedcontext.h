#pragma once

#include <KLocalizedContext>

#include <QtQml/QQmlApplicationEngine>
#include <QtQml/QQmlContext>
#include <QtQml/QQmlEngine>

namespace rust {

namespace kf6 {
void initializeEngine(QQmlEngine &engine);

}

} // namespace rust
