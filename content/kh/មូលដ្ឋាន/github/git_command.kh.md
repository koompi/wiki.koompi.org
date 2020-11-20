---
title: "ព័ត៌មានពី GitHub"
date: 2018-12-29T11:02:05+06:00
lastmod: 2020-01-05T10:42:26+06:00
draft: false
# search related keywords
keywords: ["induct", "instate"]
---
ការបង្ហាញពីការប្រើប្រាស់ Github ក្នុង ខុនសូលមាននៅក្នុងផ្នែកនេះ។ ប្រសិនបើអ្នកដែលទើបតែរៀនកូដថ្មីៗ អ្នកអាចសិក្សាពីវាបាន។

## ការតម្លើង git
យើងអាចចាប់ពីការទាញយក git មកប្រើប្រាស់ក្នុង pc របស់ជាមុន៖
```
pi -S git
```

{{< notice note >}}
កុំភ្លេចធ្វើការទាញយកversion ចុងក្រោយនៃប្រព័ន្ធប្រតិបត្តិការជាមុនដោយពាក្យបញ្ជា `pi -Syu`
{{< /notice >}}

---

## ការចម្លង repo ដែលមានហើយ
ប្រសិនបើអ្នកចង់ចម្លង  repo ដែលមានស្រាប់របស់អ្នកឬអ្នកដទៃ អ្នកអាចប្រើពាក្យបញ្ជាខាងក្រោម:
```
git clone [url] 
```

{{< notice info >}}
**Url** គឺជាអាស័យធ្ឋាននៃ World Wide Web
{{< /notice >}}

---
## ចាប់ផ្តើមគម្រោងថ្មី
ប្រសិនបើអ្នកចង់បង្កើតកំណែថ្មី អ្នកអាចដំណើរតាមការបង្ហាញ៖
```
git init
```

---
## ការកែប្រែ repository
បើអ្នកធ្លាប់បានបង្កើត repo រួច ហើយអ្នកកែប្រែបន្ទាប់ពីបានរុញទៅទុកលើ Web អ្នកអាចធ្វើតាមនេះ។
```
git add .
git commit -m " message "
```
{{< notice info >}}
ពាក្យនៅក្នុង "... " គឺជាពាក្យសម្គាល់នៅពេលការផ្លាស់ប្តូរពេលរុញទៅ web ម្តងៗ។ 
{{< /notice >}}

---
## ការបង្ហាញបច្ចុប្បន្នភាពនៃ repository
ក្នុងការបង្ហាញព័ត៌មាននៅលើ repo អ្នកត្រូវប្រើពាក្យបញ្ជានេះ
```
git status
```

---
## ការបង្កើត branch ថ្មី
នៅលើ Git មានការបែងចែក User ជាពីរ គឺ master and branch។ ដូចនេះអ្នកអាចធ្វើបង្កើត branch ដើម្បីប្រើប្រាស់ បាន ចៀសវាងការកែប្រែលើ master នាំឲ្យប៉ះពាល់ដល់ កូដដើម។
```
git branch [branchName]
```

---
## ការបង្ហាញ branches ទាំងឡាយ
ប្រសិនបើអ្នកចង់បង្ហាញ branches ទាំងអស់ដែលធ្វើការនៅ repo អ្នក។
```
git branch -a
```

---
## ការលុប branch
ប្រសិនបើអ្នកចង់ដក branch អ្នក អ្នកអាចធ្វើវាដោយដំណើរការ៖
```
git branch -d [branchName]
```

---
## ការដាក់បញ្ចូលគ្នានៃ branches
នៅពេលអ្នកធ្វើការជាក្រុម អ្នកត្រូវធ្វើការជាមួយ branch ផ្សេងៗគ្នា។ ដូច្នេះយើងអាចទាញគម្រោងរួមគ្នាជាមួយ ពាក្យបញ្ជាខាងក្រោម៖
```
git merge [branchName]
```

---
## ការមើលមាន branchនេះឫទេ?
នៅពេលដែលអ្នកចង់បង្កើត branch ថ្មី អ្នកប្រហែលជាចង់ពិនិត្យមើលថា តើbranchនោះបានបង្កើតរួចហើយឫនៅ បើមានហើយ អ្នកអាចបង្កើតbranch ផ្សេងមួយទៀតជំនួស។
```
git checkout -b [branchName]
```

ដើម្បីបង្កើត branch ថ្មីដោយមានការពិនិត្យឈ្មោះ និងបង្កើតថ្មីផងនោះ អ្នកត្រូវប្រើពាក្យបញ្ជាខាងក្រោម៖
```
git checkout -b [newBranch]
```

---
## ការបង្កើត និង លុប tag
Git tags ត្រូវបានប្រើជាចំណុចយោងនៅក្នុងលំហូរការងារអភិវឌ្ឍន៍របស់អ្នក។ អ្នកចង់បង្កើត git tags ថ្មី ដើម្បីអោយមានឯកសារយោងសំរាប់កម្មវិធីរបស់អ្នក។
```
git  tag [tagName]
```
ការលុបចោល៖
```
git tag -d [tagName]
```

---
## ការរុញ tags
ប្រសិនបើអ្នកចង់រុញ tags អ្នកអាចដំណើរការវាបាន៖
```
git push --tags
```

--
## ការទាញយកកំណៃចុងក្រោយពី repository
ដើម្បីទាញទិន្នន័យចុងក្រោយពី repo master or branch ណាមួយអ្នកអាចប្រើប្រាស់ពាក្យបញ្ជានេះបាន។
```
git pull [branchName] [remoteURl/remoteName]
```

---
## ការដាក់ remote លើ repository
យើងក៏អាចដាក់ remote លើ repoនៃ git ជាមួយនឹងបន្ទាត់ពាក្យបញ្ជានេះផងដែរ៖
```
git remote add origin [url]
```

---
## ការបង្ហាញ user លើ repo
នៅលើ git យើងអាចពិនិត្យឈ្មោះអ្នកនិពន្ធលើការ commit បាន:
```
git config --global user. name [name]
```

---
## Define the author email to be used for all commits
នៅលើ git យើងអាចពិនិត្យអ៊ីម៉ែលអ្នកនិពន្ធលើការ commit បាន:
```
git config --global user. email [email]
```

---
## ជម្រើសនៃ git
```
git help -g
```

---
## ការត្រឡប់ទៅការ commit មុន
ប្រសិនបើអ្នកចង់ត្រឡប់ការប្រព្រឹត្ដិពីមុនអ្នកអាចធ្វើវាជាមួយពាក្យបញ្ជានេះ៖
```
git revert HEAD^
```
## Forget about files that were tracked but are now in .gitignore
នេះជាពាក្យបញ្ជា៖
```
git rm -r --cached .
git add .
git commit -am " message "
```

---
## ការបញ្ជូនទៅកាន់ remote repository
យើងអាចបញ្ជូនជា branch ទៅ remote repo បានដោយ៖
```
git push [remoteURL/remoteName] [branch]
```

---
## រក្សាទុកការងារបច្ចុប្បន្នដោយគ្មានការ track
ប្រសិនបើអ្នកចង់អោយឯកសាររបស់អ្នកមិនអាច track បានអ្នកអាចធ្វើវាជាមួយ៖
```
git stash -u
```

---
## ការដោះឯកសារ untracked 
អ្នកក៏អាចយកឯកសារដែល untracked មកវិញបានដែរ។
```
git stash pop
```

---
## ការលុបឯកសារដោយ index

អ្នកអាចលុបឯកសារពី cached បានដោយ៖
```
git rm --cached [fileName]
```


ដើម្បីលុបថតឯកសារ៖
```
git rm -r --cached [directoryName]
```

---
## ការលុបឯកសារ និង ថតឯកសារដោយបង្ខំ(force)
ប្រសិនអ្នកមិនអាចលុបឯកសារខ្លះបាន អ្នកលុបដោយការបង្ខំបាន។
```
git rm -f [fileName]
```
ការលុបដោយបង្ខំ៖
```
git rm -r -f [directoryname]
```

---
## ការលុប remote branch
អ្នកអាចលុប remote branch ដោយប្រើតែពាក្យបញ្ជាមួយប៉ុណ្ណោះ។
```
git push origin :[branchName]
```

----
----
