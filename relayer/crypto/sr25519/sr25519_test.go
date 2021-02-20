// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: LGPL-3.0-only

package sr25519

import (
	"reflect"
	"testing"

	"github.com/snowfork/go-substrate-rpc-client/v2/signature"
	"github.com/snowfork/polkadot-ethereum/relayer/crypto/ss58"
)

func TestNewKeypairFromSeed(t *testing.T) {
	kp, err := NewKeypairFromSeed("//Alice", ss58.SUBSTRATE)
	if err != nil {
		t.Fatal(err)
	}

	if kp.PublicKey() == "" || kp.Address() == "" {
		t.Fatalf("key is missing data: %#v", ss58.SUBSTRATE)
	}
}

func TestKeypair_AsKeyringPair(t *testing.T) {
	kp, err := NewKeypairFromSeed("//Alice", ss58.SUBSTRATE)
	if err != nil {
		t.Fatal(err)
	}

	krp := kp.AsKeyringPair()

	// TODO: Add expected output from subkey

	if !reflect.DeepEqual(&signature.TestKeyringPairAlice, krp) {
		t.Fatalf("unexpected result.\n\tGot: %#v\n\texpected: %#v\n", krp, &signature.TestKeyringPairAlice)
	}

}

func TestEncodeAndDecodeKeypair(t *testing.T) {
	kp, err := NewKeypairFromSeed("//Alice", ss58.SUBSTRATE)
	if err != nil {
		t.Fatal(err)
	}

	enc := kp.Encode()
	res := new(Keypair)
	err = res.Decode(enc)
	if err != nil {
		t.Fatal(err)
	}

	if !reflect.DeepEqual(res, kp) {
		t.Fatalf("Fail: got %#v expected %#v", res, kp)
	}
}
