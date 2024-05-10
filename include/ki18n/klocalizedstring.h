#pragma once

#include <KLocalizedString>

namespace rust {
namespace kf6 {
void setApplicationDomain(const QByteArray &domain);

QByteArray applicationDomain();
} // namespace kf6
} // namespace rust
