---
title: "ព័ត៌មានថ្មីៗនៃ Pix"
date: 2018-12-29T11:02:05+06:00
lastmod: 2020-01-05T10:42:26+06:00
draft: false
# search related keywords
keywords: ["induct", "instate"]
---
ពេលអ្នកស្តាប់មើលទៅ`ពាក្យបញ្ជា pix`ដូចជា`ពាក្យបញ្ជា pi`អ៊ីចឹង ប៉ុន្តែការពិតទៅពួកវាមានលក្ខណះពិសេស រៀងៗខ្លួន ដោយ`ពាក្យបញ្ជា pix`តំណាងឱ្យពាក្យបញ្ជាថ្មីមួយ ដែល អនុញ្ញាតឲ្យអ្នកទាញយកកម្មវិធីជាច្រើន ទៀតដែល`ពាក្យបញ្ជា pi`មិនអាចធ្វើបាន។ 

ដូច្នេះក្រុមការងាររបស់យើងបានបង្កើតវាឡើងជាមុខងារមួយ ក្នុងករណីដែលអ្នកប្រើប្រាស់ចង់ដំឡើងកម្មវិធីមួយ ចំនួន ដែលប្រព័ន្ធប្រតិបត្តិការនៅពេលបច្ចុប្បន្ននូវមិនទាន់គាំទ្រជាផ្លូវការ។

{{< notice info >}}
ក្នុងករណីអ្នកចង់ដំឡើង **pix** ឫបានដំឡើងហើយក៏អ្នកនៅតែអាចប្រើប្រាស់ **pi** ជាធម្មតា។
{{< /notice >}}

ឯកសារលំអិតអំពី `pix` នឹងត្រូវផ្តល់ជូនអ្នកប្រើប្រាស់ទាំងអស់។ 
ជាដំបូងអ្នកត្រូវយល់ពីរបៀបដំឡើង pix ជាមុន សិនដែលបានបង្ហាញនៅខាងក្រោម។

នេះជាពាក្យបញ្ជាក្នុងការទាញប្រភពនៃ **pix**៖
```
curl -s https://repo.koompi.org/script/pix.sh -o pix && chmod +x pix && sudo mv pix /usr/bin
```

{{< notice note >}}
សូមធ្វើការពិនិត្យមើលឲ្យបានច្បាស់លាស់មុនពេលចុច Enter ហើយបើអ្នកចង់ដឹងបន្ថែមពី ខុនសូលសូមចុចនៅ[ទីនេះ](https://www.koompi.org/details/#konsole-details).
{{< /notice >}}

{{< notice tip >}}
ពួកយើងណែនាំអ្នកឱ្យធ្វើការ Copy និង Paste ដើម្បីឲ្យពាក្យបញ្ជាមានភាពត្រឹមត្រូវ។
{{< /notice >}}

ប្រសិនបើអ្នកបានដំឡើងរួចរាល់ អ្នកគ្រាន់ទាញយកកញ្ចប់ចុងក្រោយពីប្រព័ន្ធ pix ៖
```
pix update
```
ឯនេះជាពាក្យបញ្ជាមួយសម្រាប់ធ្វើការទាញយក ពាក្យបញ្ជា pi ចេញពី ពាក្យបញ្ចា pix ។
```
pix i pi
```
## តើពាក្យបញ្ជា pix អាចធ្វើអ្វីបានខ្លះ?
ពាក្យបញ្ជា pix អាចធ្វើអី្វដែលមិនអាចធ្វើបានអាចធ្វើបាន ហើយអ្នកអាចស្វែងរកព័ត៌មានបន្ថែមពីវាបាន ដោយ ការវាតបញ្ចូលពាក្យបញ្ជាខាងក្រោមក្នុង ខុនសូល។
```
pix
```

## ការឆេកមើលកម្មវិធីនៅលើ Pix

នៅក្នុង**pix version**, អ្នកអាចរកមើលកម្មវិធីទាំងនោះបានដោយប្រើ `-l`ពីក្រោយពាក្យបញ្ជា pix:
```
pix -l
```
ឯនេះជាលទ្ធផលពីការប្រើពាក្យបញ្ជាខាងលើ៖
```
NO             APPLICATION           INSTALL                      REMOVE     

1              adobe-acrobat-xi      pix -i adobe-acrobat-xi      pix -r adobe-acrobat-xi     
2              adobe-photoshop-cc    pix -i adobe-photoshop-cc    pix -r adobe-photoshop-cc   
3              asc-timetables        pix -i asc-timetables        pix -r asc-timetables       
4              chinese-support       pix -i chinese-support       pix -r chinese-support      
5              flutter               pix -i flutter               pix -r flutter              
6              ios-support           pix -i ios-support           pix -r ios-support          
7              khmer-typing          pix -i khmer-typing          pix -r khmer-typing         
8              koompi-academy        pix -i koompi-academy        pix -r koompi-academy       
9              koompi-themes         pix -i koompi-themes         pix -r koompi-themes        
10             master-pdf-editor5    pix -i master-pdf-editor5    pix -r master-pdf-editor5   
11             ms-office-2013        pix -i ms-office-2013        pix -r ms-office-2013       
12             pi                    pix -i pi                    pix -r pi  
```
## ការដំឡើង Version Pix
ដូចដែលធ្លាប់បានបង្ហាញពីការដំឡើង Versionម្តងនៅខាងលើហើយ អ្នកក៏អាចប្រើប្រាស់ពាក្យបញ្ជាកាត់ ក្នុងការដំឡើង Versionផងដែរ 
```
pix u
```
{{< notice note >}}
បើអ្នកបានដំឡើង Versionចុងក្រោយវារួចរាល់ វានឹងបង្ហាញដូចខាងក្រោម។
{{< /notice >}}

```
[ok] No Problems Found.

Dependencies Found:     2
Dependencies Missing:   0

No app updates available.
```

## ការបញ្ចូល និងលុបចោលលើ Pix Version
ក្នុងPix Version អ្នកអាចបញ្ចូលកម្មវិធីបានដោយរបៀបនេះ
```
pix i <App_Name>
```

ឯការលុបចោលគឺធ្វើតាមរបៀបនេះ៖
```
pix r <App_Name>
```
- `i` មានន័យថាដំឡើង ឫបញ្ចូល (Install)
- `r` មានន័យថាលុបបំបាត់ (Remove)

ការបង្ហាញពីការលុប និងដំឡើងកម្មវិធី Microsoft Office 2013៖
{{% tabs %}}
  {{% tab "ការដំឡើង" %}}
   ```
   pix i ms-office-2013 
   ```
  {{% /tab %}}

  {{% tab "ការលុប" %}}
  ```
  pix r ms-office-2013 
  ```
  {{% /tab %}}
{{% tabs %}}

----
----