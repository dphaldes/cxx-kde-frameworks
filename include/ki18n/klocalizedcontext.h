#pragma once

#include <KLocalizedContext>

#include <QQmlContext>
#include <QtQml/QQmlApplicationEngine>
#include <QtQml/QQmlEngine>

namespace rust {

namespace kf6 {
void initFromEngine(QQmlEngine &engine);

}

} // namespace rust
