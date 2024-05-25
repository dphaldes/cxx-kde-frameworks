#pragma once

#include <KAboutData>


#include "rust/cxx.h"

namespace rust {
namespace kf6 {

auto from(QString componentName, QString displayName, QString version,
          QString shortDescription, int license) -> std::unique_ptr<KAboutData>;

void setApplicationData(const KAboutData &aboutData);
} // namespace kf6
} // namespace rust
