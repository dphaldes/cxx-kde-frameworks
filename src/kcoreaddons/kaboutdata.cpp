#include "kf6/kaboutdata.h"

#include <iostream>

namespace rust {
namespace kf6 {

auto from(QString componentName, QString displayName, QString version,
          QString shortDescription, int license)
    -> std::unique_ptr<KAboutData> {
  return std::make_unique<KAboutData>(
      componentName, displayName, version, shortDescription,
      static_cast<KAboutLicense::LicenseKey>(license));
}

void setApplicationData(const KAboutData &aboutData) {
  KAboutData::setApplicationData(aboutData);
}

void registerQmlSingletonType(rust::String uri, int version_major,
                              int version_minor, rust::String type_name) {
  qmlRegisterSingletonType(
      uri.c_str(), version_major, version_minor, type_name.c_str(),
      [](QQmlEngine *engine, QJSEngine *) -> QJSValue {
        return engine->toScriptValue(KAboutData::applicationData());
      });
}
} // namespace kf6
} // namespace rust
