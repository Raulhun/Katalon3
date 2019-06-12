import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import internal.GlobalVariable as GlobalVariable
import com.kms.katalon.core.webui.driver.DriverFactory as DriverFactory
import org.openqa.selenium.By as By
import org.openqa.selenium.WebElement as WebElement
import com.kms.katalon.core.annotation.Keyword as Keyword
import org.openqa.selenium.WebDriver as WebDriver
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUiBuiltInKeywords
import com.kms.katalon.core.util.KeywordUtil as KeywordUtil
import org.openqa.selenium.Keys as Keys
import org.openqa.selenium.interactions.Actions as Actions

'Belepes a beallitasok menube'
WebUI.click(findTestObject('Vezerlopult/btn_Beallitasok_sidebaron'))

WebUI.waitForElementVisible(findTestObject('Vezerlopult/Beallitasok/btn_Mentes_Bealitasokban'), 30)

'1. Egyedi mezők gomb click'
WebUI.click(findTestObject('Vezerlopult/Egyedi_mezok/btn_Egyedi_mezok'))

WebUI.waitForElementVisible(findTestObject('Vezerlopult/Egyedi_mezok/btn_Torzsadatok'), 30)

WebUI.click(findTestObject('Vezerlopult/Egyedi_mezok/btn_Torzsadatok'))

WebUI.waitForElementVisible(findTestObject('Vezerlopult/Egyedi_mezok/btn_Mentes'), 30)

WebUI.click(findTestObject('Vezerlopult/Egyedi_mezok/btn_Egyedi_mezok'))

WebUI.verifyElementVisible(findTestObject('Vezerlopult/Egyedi_mezok/btn_Torzsadatok'))

WebUI.click(findTestObject('Vezerlopult/Egyedi_mezok/btn_Uj_lapful_hozaad_Pacienstorzs'))

WebUI.waitForElementVisible(findTestObject('Vezerlopult/Egyedi_mezok/btn_Mentes'), 30)

WebUI.click(findTestObject('Vezerlopult/Egyedi_mezok/btn_Egyedi_mezok'))

WebUI.verifyElementVisible(findTestObject('Vezerlopult/Egyedi_mezok/btn_Torzsadatok'))

WebUI.click(findTestObject('Vezerlopult/Egyedi_mezok/btn_Vizsgalat'))

WebUI.waitForElementVisible(findTestObject('Vezerlopult/Egyedi_mezok/btn_Mentes'), 30)

WebUI.click(findTestObject('Vezerlopult/Egyedi_mezok/btn_Egyedi_mezok'))

WebUI.verifyElementVisible(findTestObject('Vezerlopult/Egyedi_mezok/btn_Torzsadatok'))

WebUI.click(findTestObject('Vezerlopult/Egyedi_mezok/btn_Uj_lapful_hozzaadasa_Vizsg_lapok'))

WebUI.waitForElementVisible(findTestObject('Vezerlopult/Egyedi_mezok/btn_Mentes'), 30)

WebUI.click(findTestObject('Vezerlopult/Egyedi_mezok/btn_Egyedi_mezok'))

WebUI.verifyElementVisible(findTestObject('Vezerlopult/Egyedi_mezok/btn_Torzsadatok'))

'2. Elofizetes click'
WebUI.click(findTestObject('Vezerlopult/Egyedi_mezok/btn_Elofizetes'))

WebUI.waitForElementVisible(findTestObject('Vezerlopult/table_Arak_Elofizetesben'), 30)

'3. Sugo click'

WebUI.click(findTestObject('Vezerlopult/btn_Sugo_sidebar'))

WebUI.waitForElementVisible(findTestObject('Vezerlopult/page_Sugo'), 30)

WebUI.click(findTestObject('Vezerlopult/btn_Beallitasok_sidebaron'))

WebUI.waitForElementVisible(findTestObject('Vezerlopult/Beallitasok/btn_Mentes_Bealitasokban'), 30)


