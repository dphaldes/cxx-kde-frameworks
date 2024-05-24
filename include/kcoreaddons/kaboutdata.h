#pragma once

#include <KAboutData>
#include <QQmlEngine>
#include <QString>

#include "rust/cxx.h"

namespace rust {
namespace kf6 {

auto from(QString componentName, QString displayName, QString version,
          QString shortDescription, int license) -> std::unique_ptr<KAboutData>;

void setApplicationData(const KAboutData &aboutData);

void registerQmlSingletonType(rust::String uri, int version_major,
                              int version_minor, rust::String type_name);
} // namespace kf6
} // namespace rust
