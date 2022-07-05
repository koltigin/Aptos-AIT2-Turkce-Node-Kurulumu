<a href="https://aptos.dev">
	<img width="100%" src="./.assets/aptos_banner.png" alt="Aptos Banner" />
</a>

---

[![Aptos Rust Crate Documentation (main)](https://img.shields.io/badge/docs-main-59f)](https://aptos-labs.github.io/aptos-core/)
[![License](https://img.shields.io/badge/license-Apache-green.svg)](LICENSE)
[![CircleCI](https://circleci.com/gh/aptos-labs/aptos-core/tree/main.svg?style=shield&circle-token=d248cf1c0580eb69a507a71c0d238e1eaf193767)](https://circleci.com/gh/aptos-labs/aptos-core/tree/main)
[![grcov](https://img.shields.io/badge/Coverage-grcov-green)](https://ci-artifacts.aptoslabs.com/coverage/unit-coverage/latest/index.html)
[![test history](https://img.shields.io/badge/Test-History-green)](https://ci-artifacts.aptoslabs.com/testhistory/aptos/aptos/auto/ci-test.yml/index.html)
[![Automated Issues](https://img.shields.io/github/issues-search?color=orange&label=Automated%20Issues&query=repo%3Aaptos%2Faptos%20is%3Aopen%20author%3Aapp%2Fgithub-actions)](https://github.com/aptos-labs/aptos-core/issues/created_by/app/github-actions)
[![Discord chat](https://img.shields.io/discord/945856774056083548?style=flat-square)](https://discord.gg/aptoslabs)

# Aptos Incentivized Testnet 2 Türkçe Node Kurulumu

## Takvim
* Kayıtların başlangıcı: 30 Haziran 2022
* Topluluk Oylama Süreci: 5 Temmuz 2022
* Kayıtların bitişi: 7 Temmuz 2022
* Kayıt bildirimlerinin gönderilmesi: 11 Temmuz 2022
* Testnet 2'nin başlaması: 12 Temmuz 2022
* Testnet 2'nin bitişi:  22 Temmuz 2022

## Sistem gereksinimleri (Minimum)
* CPU: 4vcpu
* Ram: 8GB RAM
* SSD: 300GB 
* İşletim sistemi: Ubuntu 20.04

## Kurulum

### Gerekli Güncelemelerin ve Kütüphanelerin Kurulması
```
sudo su
cd $HOME
sudo apt update && sudo apt upgrade -y
```
```
sudo apt install curl tar wget clang pkg-config libssl-dev jq build-essential bsdmainutils git make ncdu gcc git jq chrony unzip liblz4-tool -y
```

### Docker Kurulumu
```
curl -fsSL https://get.docker.com -o get-docker.sh
sudo sh get-docker.sh
```

### Docker Compose Kurulumu
```
curl -SL https://github.com/docker/compose/releases/download/v2.5.0/docker-compose-linux-x86_64 -o /usr/local/bin/docker-compose
sudo chmod +x /usr/local/bin/docker-compose
sudo ln -s /usr/local/bin/docker-compose /usr/bin/docker-compose
```

### Aptos Kurulumu

```
wget -qO aptos-cli.zip https://github.com/aptos-labs/aptos-core/releases/download/aptos-cli-0.2.0/aptos-cli-0.2.0-Ubuntu-x86_64.zip
unzip -o aptos-cli.zip
chmod +x aptos
mv aptos /usr/local/bin
```

### Aptos Klasörü Oluşturma

```
export WORKSPACE=aptos-test
mkdir ~/$WORKSPACE
cd ~/$WORKSPACE
```

### validator.yaml ve docker-compose.yaml Dosyalarını İndirme 

```
wget -O $HOME/$WORKSPACE/docker-compose.yaml https://raw.githubusercontent.com/aptos-labs/aptos-core/main/docker/compose/aptos-node/docker-compose.yaml
wget -O $HOME/$WORKSPACE/validator.yaml https://raw.githubusercontent.com/aptos-labs/aptos-core/main/docker/compose/aptos-node/validator.yaml
```

### Çalışma Dizininizde Anahtar Çiftleri (node owner key, consensus key and networking key) Oluşturma
```
aptos genesis generate-keys --assume-yes --output-dir $HOME/$WORKSPACE
```

### Validator Bilgilerini Yapılandırma 
IP değişkenine node ip ve portumuzu ilişkilendiriyoruz.
*NODE_ADINIZ* bölümünü node adınız ne olacaksa onu yazıyoruz.
```
IP=$(curl icanhazip.com):6180
cd ~/$WORKSPACE
aptos genesis set-validator-configuration \
    --keys-dir ~/$WORKSPACE --local-repository-dir ~/$WORKSPACE \
    --username NODE_ADINIZ \
    --validator-host $IP
```

Yukarıdaki kod sonrasında NODE_ADINIZ.yaml şeklinde bir dosya oluşacak.
Bu dosya içeriğini aşağıdaki kodla kontrol ediyoruz.
```
cat $HOME/$WORKSPACE/NODE_ADINIZ.yaml
```

Dosya içeriği aşağıdakine benzer şekilde ise sıkıntı yoktur. Keylerinizi kayıt olurken kullanacaksınız.

```
account_address: 7410973313fd0b5c69560fd8cd9c4aaeef873f869d292d1bb94b1872e737d64f
consensus_public_key: "0x4e6323a4692866d54316f3b08493f161746fda4daaacb6f0a04ec36b6160fdce"
account_public_key: "0x83f090aee4525052f3b504805c2a0b1d37553d611129289ede2fc9ca5f6aed3c"
validator_network_public_key: "0xa06381a17b090b8db5ffef97c6e861baad94a1b0e3210e6309de84c15337811d"
validator_host:
  host: 35.232.235.205
  port: 6180
stake_amount: 1
```

### layout.yaml Dosyası Oluşturma
*NODE_ADINIZ* kısmına yukarıda yazdığımız node adımızı yazıyoruz.
```
echo "---
root_key: \"F22409A93D1CD12D2FC92B5F8EB84CDCD24C348E32B3E7A720F3D2E288E63394\"
users:
  - \"NODE_ADINIZ\"
chain_id: 40
min_stake: 0
max_stake: 100000
min_lockup_duration_secs: 0
max_lockup_duration_secs: 2592000
epoch_duration_secs: 86400
initial_lockup_timestamp: 1656615600
min_price_per_gas_unit: 1
allow_new_validators: true" >layout.yaml
```

### AptosFramework Move bytecode Dosyasını İndirme

```
wget https://github.com/aptos-labs/aptos-core/releases/download/aptos-framework-v0.2.0/framework.zip
unzip framework.zip
```

### Genesis blob ve waypoint Derlemesi Yapma
```
aptos genesis generate-genesis --local-repository-dir ~/$WORKSPACE --output-dir ~/$WORKSPACE
```

### Docker Başlatma
```
docker-compose down -v
docker-compose up -d
```

### Node Durumunu Kontrol Etmek

API kısmını 80 olarak değiştiriniz
https://aptos-node.info/

## KYC Yapma
https://community.aptoslabs.com/it2

### Key Bilgilerini Görme 
Yukarıda bu dosyayı oluştururken *NODE_ADINIZ* kısmına node adımızı ne yazdıysak onu yazıp. Dosya içeriğine bakıyoruz.
Buradaki bilgileri KYC yaparken gireceğiz. Full Node Public Key bölümünü boş bırakıyoruz, çünkü bu opsiyonel bir seçenek olduğundan bir burada full node kurmadık.

```
cat ~/$WORKSPACE/NODE_ADINIZ.yaml
```


## Kaynaklar

* [Aptos Labs](https://aptoslabs.com/)
* [Aptos Developer Network](https://aptos.dev)
* [Getting Started](https://aptos.dev/guides/getting-started)
* [Life of a Transaction](https://aptos.dev/guides/basics-life-of-txn)
* [Aptos Discord](https://discord.gg/aptoslabs).

## Katkıda Bulunma

 [Katkıda Bulunma Rehberimizi](https://github.com/aptos-labs/aptos-core/blob/main/CONTRIBUTING.md) ve [Davranış Kurallarımızı](https://github.com/aptos-labs/aptos-core/blob/main/CODE_OF_CONDUCT.md) okuyarak Aptos projesine katkıda bulunma hakkında daha fazla bilgi edinebilirsiniz.


Aptos Core is licensed as [Apache 2.0](https://github.com/aptos-labs/aptos-core/blob/main/LICENSE).
