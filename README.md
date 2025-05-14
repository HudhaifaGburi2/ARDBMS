
# نظام إدارة قواعد البيانات العلائقية العربي (ARDBMS)



## 🎯 الهدف
نظام إدارة قواعد البيانات العلائقية العربي (ARDBMS) هو نظام مفتوح المصدر تم تطويره لأغراض تعليمية، يهدف إلى:
- توفير مورد تعليمي عالي الجودة باللغة العربية
- شرح مفاهيم أنظمة قواعد البيانات من خلال كود مصدري حقيقي
- تشجيع المطورين العرب على المساهمة في البرمجيات الحرة

## 📜 الترخيص
هذا المشروع مرخص تحت [رخصة جنو العمومية العامة v3.0](LICENSE). هذا يعني:
- يمكنك استخدام، تعديل، وتوزيع النظام بحرية
- أي أعمال مشتقة يجب أن تبقى مفتوحة المصدر تحت نفس الرخصة
- يجب ذكر التعديلات التي تقوم بها

## ✨ لماذا تشارك؟
- **تعليمي**: فرصة لفهم أنظمة قواعد البيانات بشكل عميق
- **مجتمعي**: المساهمة في بناء أدوات عربية حرة
- **مهني**: تطوير مهاراتك في أنظمة قواعد البيانات

## 🛠 كيفية المساهمة
1. انسخ المستودع (`git clone`)
2. اختر إحدى المهام:
   - تصحيح أخطاء (ابحث عن التسمية `خطأ`)
   - تحسين أداء (ابحث عن التسمية `تحسين أداء`)
   - تطوير وثائق (ابحث عن التسمية `توثيق`)
3. أرسل طلب دمج (Pull Request)

## 📂 هيكل المشروع
```
ardbms/
├── core/           # النواة الأساسية
├── parser/         # محلل الاستعلامات
├── storage/        # نظام التخزين
├── docs/           # وثائق عربية
└── tests/          # اختبارات النظام
```

## 💻 البدء السريع
```bash
# تثبيت المتطلبات
sudo apt install build-essential flex bison

# بناء المشروع
make && make test

# تشغيل الواجهة التفاعلية
./bin/ardbms-cli
```

## 🌍 المجتمع
- [قناة التليجرام](https://t.me/ardbms_community)
- [مجموعة الديسكورد](https://discord.gg/example)
- لقاءات أسبوعية عبر زووم (تفاصيل في الويكي)

---

# Arabic Relational Database Management System (ARDBMS)

![ARDBMS Logo](https://via.placeholder.com/150) <!-- Replace with actual logo -->

## 🎯 Purpose
ARDBMS is an open-source RDBMS developed for educational purposes, aiming to:
- Provide high-quality Arabic educational resources
- Teach database system concepts through real source code
- Encourage Arab developers to contribute to free software

## 📜 License
This project is licensed under the [GNU General Public License v3.0](LICENSE). This means:
- You can freely use, modify, and distribute the system
- Any derivative works must remain open-source under same license
- You must disclose any modifications you make

## ✨ Why Contribute?
- **Educational**: Deepen your understanding of database systems
- **Community**: Contribute to Arabic free software tools
- **Professional**: Develop your database systems skills

## 🛠 How to Contribute
1. Clone the repository
2. Pick a task:
   - Bug fixes (look for `bug` label)
   - Performance improvements (look for `performance` label)
   - Documentation (look for `documentation` label)
3. Submit a Pull Request

## 📂 Project Structure
```
ardbms/
├── core/           # Core engine
├── parser/         # Query parser
├── storage/        # Storage system
├── docs/           # Arabic documentation
└── tests/          # System tests
```

## 💻 Quick Start
```bash
# Install dependencies
sudo apt install build-essential flex bison

# Build project
make && make test

# Run interactive CLI
./bin/ardbms-cli
```

## 🌍 Community
soon
- Discord Group
```
